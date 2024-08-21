# Serverless HTTP Server

## Description

This repository is a personal sandbox to play around serverless design using AWS, Rust, Axum, HTMX and Just.
I play with it in my free time, it is not perfect and that is the goal: to play with ideas and improve them as possible.

### Why Axum?

Previously, i crafted a custom router used in a Lambda integrated with a API GW in a HTTP proxy configuration.
Then i realized that i should not reinvent the wheel: Axum is pretty powerful and provides a nice router, middleware, context and other capabilities.

### Why Rust?

In my opinion, Rust is an awesome language with powerful capabilities and wonderful ecosystem.
Of course, it enforces security and good practices, plus delivers pretty fast executables.
In a serverless environment, specially a Lambda function, this means no cold starts, super fast durations and cheap costs as a consequence.
Today, you can have tracing and eveything else you would have in standard languages for such context.

### Why AWS?

I work with AWS on a daily basis and they have pretty good cloud solutions.
This personal sandbox aims to play around and replace an exisiting web site and api that i developed in Go.
Today this solution is on a EC2 container which costs me around 10 EUR/month. If i switch to Lambdas it could cost me nothing, because it a very low usage.
Lambdas are in my opinion more suitable for async tasks, not for an API/WEB server. But for my personal needs it will save me some money.

### Why HTMX?

Yeah, this one is very controversial, but i feel that HTMX can improve a web site performance, plus make it easier to add more features and maintenance.
HTMX is the best HATEOAS approach which challenge some established standards, but i think we should be open minded and try to break paradigms!

### Why Just?

It is simple, clean and provides what i need.

## How to use

You can deploy it locally using Localstack or in AWS. For both you need AWS CDK installed.

### Deploy locally

You need to start Localstack on your machine. I provide a docker compose for that in the `local` folder.

You also need `cdklocal` installed. Then type:
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
