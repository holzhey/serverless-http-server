import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { join } from 'path';
import { CfnOutput } from 'aws-cdk-lib';
import { RustFunction } from 'cargo-lambda-cdk';
import { LambdaRestApi } from 'aws-cdk-lib/aws-apigateway'
import { FunctionUrlAuthType } from "aws-cdk-lib/aws-lambda";

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // TODO: do not use RustFunction, switch to usual build and CDK constructs.
    const handler = new RustFunction(this, 'Axum API', {
      manifestPath: join(__dirname, '..', '..'),
    });

   const lambdaUrl = handler.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    });
    new CfnOutput(this, 'Axum FunctionUrl ', { value: lambdaUrl.url });
    
    new LambdaRestApi(this, 'axum', { handler });
  }
}
