# Serverless HTTP Server

## Description

This repository is a personal sandbox to play around serverless design using AWS, Rust, Axum, HTMX and Just.
I play with it in my free time, it is not perfect and that is the goal: to play with ideas and improve them as possible.
There are some features in my mind (see below), but i also want to refactor existing code to a well organized version.
I also need to cover everything with tests, but for now i am mainly brainstorming.

### Why [Axum](https://github.com/tokio-rs/axum)?

Previously, i crafted a custom router used in a Lambda integrated with a API GW in a HTTP proxy configuration.
Then i realized that i should not reinvent the wheel: Axum is pretty powerful and provides a nice router, middleware, context and other capabilities.
Axum is part of the [Tokio](https://tokio.rs/) ecosystem, an asynchronous runtime for the Rust programming language.

### Why [Rust](https://www.rust-lang.org/)?

In my opinion, Rust is an awesome language with powerful capabilities and wonderful ecosystem.
It is complex? Absolutely, i can barely understand some macro implementation. But when well used is absolutely awesome, readable, extendable and performant.
In a serverless environment, specially a Lambda function, this means no cold starts, super fast durations and cheap costs as a consequence.
Today, you can have tracing and eveything else you would have in standard languages for such context.

### Why [AWS](https://aws.amazon.com/)?

I work with AWS on a daily basis and they have amazing cloud solutions.
This personal sandbox aims to play around and replace an exisiting web site and api that i developed in Go.
Today this solution is on a EC2 container which costs me around 10 EUR/month. If i switch to Lambdas it could cost me nothing, because it a very low usage.
Lambdas are in my opinion more suitable for async tasks, not for an API/WEB server. But for my personal needs it will save me some money.

### Why [HTMX](https://htmx.org/)?

Yeah, this one is very controversial, but i feel that HTMX can improve a web site performance, plus make it easier to add more features and maintenance.
HTMX is the best HATEOAS approach which challenge some established standards, but i think we should be open minded and try to break paradigms!

### Why [Just](https://just.systems/)?

It is simple, clean and provides what i need.

### Why [Helix](https://helix-editor.com/)?

Well, i did not mentioned it but this is the editor i use personally, to develop in Rust, CDK, etc.
It is an amazing editor writen in Rust, blazing fast (try to open a huge CSV file with Helix then with NeoVim and compare the performance) and simple to use but with powerful resources.

## How to use

You need to install Just and [AWS CDK](https://aws.amazon.com/cdk/) (properly configured to your AWS account, of course).
You can deploy it locally using Localstack or in AWS. The deployments covered here will provide you with the URL of the API GW, then you can use it in a browser.
For now, there is not too much to look, but take a look on the duration of the lambda and HTTP requests. :-)
I will evolve it to at least a form where you can add items to a DynamoDB table, protected by a Captcha.
I should also add some CDK configuration to allow you to define an domain hosted on Route53 to be used and add a subdomain on top of it with a SSL certificate, so it will be nicer to play with the solution.

### Deploy locally

You need to start Localstack on your machine. I provide a docker compose for that in the `local` folder.

You also need [`cdklocal`](https://github.com/localstack/aws-cdk-local) installed. Then type:
```
just deploy-local
```

#### You can watch the logs locally by typing:
```
just watch-local-service-logs
```

### Deploy on AWS

"Just" type:
```
just deploy
```

### Destroy on AWS

"Just" type:
```
just destroy
```
