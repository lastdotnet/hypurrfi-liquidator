import { IgnoreMode } from "aws-cdk-lib";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as iam from "aws-cdk-lib/aws-iam";
import * as logs from "aws-cdk-lib/aws-logs";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

export type BackendProps = {
  cluster: ecs.Cluster;
  vpc: ec2.Vpc;
  service: string;
  healthCheckPath: string;
  containerEnvironment?: { [key: string]: string };
  containerSecrets?: { [key: string]: ecs.Secret };
  inlineRolePolicies?: iam.RoleProps["inlinePolicies"];
};

export class Backend extends Construct {
  public readonly service: ecs.FargateService;
  public readonly role: iam.Role;

  constructor(scope: Construct, id: string, props: BackendProps) {
    super(scope, id);

    const { cluster, vpc, inlineRolePolicies = {} } = props;

    const liquidatorSecret = secretsmanager.Secret.fromSecretNameV2(
      this,
      "liquidator-secret",
      "aave-liquidator/config"
    );

    this.role = new iam.Role(this, `${id}-role`, {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
      managedPolicies: [
        iam.ManagedPolicy.fromAwsManagedPolicyName(
          "service-role/AmazonECSTaskExecutionRolePolicy"
        ),
      ],
      inlinePolicies: {
        ...inlineRolePolicies,
      },
    });

    const taskDefinition = new ecs.TaskDefinition(this, "api", {
      family: props.service,
      compatibility: ecs.Compatibility.EC2_AND_FARGATE,
      cpu: "1024",
      memoryMiB: "2048",
      networkMode: ecs.NetworkMode.AWS_VPC,
      taskRole: this.role,
    });

    taskDefinition.addToTaskRolePolicy(
      new iam.PolicyStatement({
        actions: ["secretsmanager:GetSecretValue"],
        resources: [liquidatorSecret.secretArn],
      })
    );

    const image = ecs.ContainerImage.fromAsset("./", {
      file: "Dockerfile",
      buildArgs: {
        SERVICE: props.service,
      },
      ignoreMode: IgnoreMode.DOCKER,
    });

    taskDefinition.addContainer("backend", {
      image,
      memoryLimitMiB: 2048,
      environment: {
        ...props.containerEnvironment,
        BID_PERCENTAGE: "100",
        DEPLOYMENT: "seashell",
      },
      secrets: {
        LIQUIDATOR_CONFIG: ecs.Secret.fromSecretsManager(liquidatorSecret),
      },
      command: [
        "/app/aave-v3-liquidator",
        "--rpc",
        "${LIQUIDATOR_CONFIG.rpc}",
        "--private-key",
        "${LIQUIDATOR_CONFIG.privateKey}",
        "--bid-percentage",
        "100",
        "--deployent",
        "seashell",
        "--liquidator-address",
        "${LIQUIDATOR_CONFIG.liquidatorAddress}",
      ],
      portMappings: [{ containerPort: 3000 }],
      logging: ecs.LogDriver.awsLogs({
        streamPrefix: "hypurr-liquidator-backend",
        logRetention: logs.RetentionDays.THREE_DAYS,
      }),
      essential: true,
    });

    this.service = new ecs.FargateService(this, `${id}-service`, {
      cluster: cluster,
      taskDefinition: taskDefinition,
      assignPublicIp: false,
      minHealthyPercent: 0,
      desiredCount: 1,
      circuitBreaker: {
        rollback: true,
      },
    });
  }
}
