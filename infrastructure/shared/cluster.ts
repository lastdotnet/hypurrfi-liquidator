import * as cdk from "aws-cdk-lib";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecs from "aws-cdk-lib/aws-ecs";

type Props = {
  vpc: ec2.IVpc;
};

export class Cluster extends cdk.Stack {
  public readonly cluster: ecs.Cluster;

  constructor(scope: cdk.App, id: string, { vpc }: Props) {
    super(scope, id, {
      description: id,
    });

    this.cluster = new ecs.Cluster(this, "hypurr-liquidator-cluster", {
      vpc,
      containerInsights: true,
      clusterName: "hypurr-liquidator-cluster",
    });
  }
}
