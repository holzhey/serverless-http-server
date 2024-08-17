import * as cdk from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import { Construct } from 'constructs';
import { CfnOutput } from 'aws-cdk-lib';
import { FunctionUrlAuthType } from "aws-cdk-lib/aws-lambda";
import { RetentionDays } from "aws-cdk-lib/aws-logs";

export class ServiceStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const serviceHandler = new lambda.Function(this, "ApiService", {
      functionName: "ApiServiceHandler",
      code: lambda.Code.fromAsset(
        "../service/target/lambda/service"
      ),
      runtime: lambda.Runtime.PROVIDED_AL2023,
      architecture: lambda.Architecture.X86_64,
      handler: "not.required",
      memorySize: 128,
      logRetention: RetentionDays.FIVE_DAYS,
      timeout: cdk.Duration.seconds(5),
      environment: {
        AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH: "true",
        HTMX_URL: "https://unpkg.com/htmx.org@1.9.12",
        HTMX_INTEGRITY: "sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2",
      },
    });

    const lambdaUrl = serviceHandler.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    });
    new CfnOutput(this, 'Lambda FunctionUrl ', { value: lambdaUrl.url });
    
    const api = new apigateway.RestApi(this, "ServiceGateway", {
        restApiName: "ServiceApiGateway",
        deployOptions: {
          stageName: "current",          
        },
    }) ;

      const proxyResource = api.root.addResource("{proxy+}");
    
    api.root.addMethod(
      "ANY",
      new apigateway.LambdaIntegration(serviceHandler, { proxy: true })
    );

    proxyResource.addMethod(
      "ANY",
      new apigateway.LambdaIntegration(serviceHandler, { proxy: true })
    );
  }
}
