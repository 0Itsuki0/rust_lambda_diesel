import { join } from 'path';
import { RustFunction } from 'cargo-lambda-cdk';
import { EndpointType, LambdaRestApi } from 'aws-cdk-lib/aws-apigateway'
import { RemovalPolicy, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";


export class CdkStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const lmabdaHandler = new RustFunction(this, 'RustLambdaDemoFunction', {
            // Path to the root directory.
            manifestPath: join(__dirname, '..', '..'),
            environment: {
                "DATABASE_URL": "your_database_url"
            }
        });
        lmabdaHandler.applyRemovalPolicy(RemovalPolicy.DESTROY)


        const restApi = new LambdaRestApi(this, 'RustLambdaDemoAPIGateway', {
            handler: lmabdaHandler,
            endpointTypes: [EndpointType.REGIONAL]
        });
        restApi.applyRemovalPolicy(RemovalPolicy.DESTROY)
    }
}