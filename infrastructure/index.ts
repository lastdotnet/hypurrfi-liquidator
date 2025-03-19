import { App } from "aws-cdk-lib";
import { Cluster } from "./shared/cluster";
import { Network } from "./shared/network";
import { Liquidator } from "./liquidator";

const app = new App();

const network = new Network(app, "hypurr-liquidator-network", {});

const cluster = new Cluster(app, "hypurr-liquidator-cluster", {
  vpc: network.vpc,
});

new Liquidator(app, "hypurr-liquidator-backend", {
  vpc: network.vpc,
  cluster: cluster.cluster,
  policies: {},
});
