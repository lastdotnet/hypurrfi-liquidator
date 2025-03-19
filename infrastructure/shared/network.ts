import * as cdk from "aws-cdk-lib";
import * as ec2 from "aws-cdk-lib/aws-ec2";

export interface NetworkProps extends cdk.StackProps {}

export class Network extends cdk.Stack {
  public readonly vpc: ec2.Vpc;
  public readonly clusterSecurityGroup: ec2.SecurityGroup;
  public readonly rdsSecurityGroup: ec2.SecurityGroup;
  public readonly redisSecurityGroup: ec2.SecurityGroup;

  constructor(scope: cdk.App, id: string, props: NetworkProps) {
    super(scope, id, props);

    this.vpc = new ec2.Vpc(this, "hypurr-liquidator-vpc", {
      maxAzs: 2,
      natGateways: 1,
      enableDnsHostnames: true,
      enableDnsSupport: true,
      subnetConfiguration: [
        {
          name: "public",
          subnetType: ec2.SubnetType.PUBLIC,
        },
        {
          name: "private-egress",
          subnetType: ec2.SubnetType.PRIVATE_WITH_EGRESS,
        },
      ],
    });

    this.clusterSecurityGroup = new ec2.SecurityGroup(
      this,
      "liquidator-cluster-security-group",
      {
        vpc: this.vpc,
        allowAllOutbound: true,
      }
    );

    this.clusterSecurityGroup.addIngressRule(
      ec2.Peer.ipv4("0.0.0.0/0"),
      ec2.Port.allTraffic(),
      "Cluster"
    );
  }
}
