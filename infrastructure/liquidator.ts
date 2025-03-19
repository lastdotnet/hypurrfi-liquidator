import * as cdk from "aws-cdk-lib";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as iam from "aws-cdk-lib/aws-iam";

import { Backend } from "./constructs/backend";

export interface APIProps extends cdk.StackProps {
  vpc: ec2.Vpc;
  cluster: ecs.Cluster;
  policies?: iam.RoleProps["inlinePolicies"];
}

export class Liquidator extends cdk.Stack {
  constructor(scope: cdk.App, id: string, props: APIProps) {
    super(scope, id, props);

    const { vpc, cluster } = props;

    new Backend(this, "hypurr-liquidator", {
      cluster,
      vpc,
      service: "liquidator",
      inlineRolePolicies: props.policies,
      healthCheckPath: "/health",
    });
  }
}
