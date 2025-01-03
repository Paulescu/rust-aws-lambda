<div align="center">
    <h1>Build and deploy a serverless API with Rust and AWS Lambda</h1>
</div>

In this project you will learn how to build a REST API in Rust and deploy it to AWS Lambda.

No more.
No less.

<div align="center">
    <img src="./media/ferris_to_the_moon.webp" width='250' />
</div>


### Table of contents
1. [Why Rust and AWS Lambda are a match made in heaven?](#why-rust-and-aws-lambda-are-a-match-made-in-heaven)
2. [Our dataset](#our-dataset)
3. [The tools we will use](#the-tools-we-will-use)
4. [Bootstrapping the project](#bootstrapping-the-project)
5. [Developing the API](#developing-the-api)
6. [Running the API locally](#running-the-api-locally)
7. [Testing the API](#testing-the-api)
8. [Building the API binary](#building-the-api-binary)
8. [Deploying the API to AWS Lambda](#deploying-the-api-to-aws-lambda)
9. [Invoking the API](#invoking-the-api)
8. [Wanna learn more Real World ML engineering with me?](#wanna-learn-more-real-world-ml-engineering-with-me)


## Why Rust and AWS Lambda are a match made in heaven?

Rust is a systems programming language that is fast, safe, and easy to use.
I won't lie to you. It is harder to learn than Python or JavaScript.
But once you get the hang of it, you will be able to build
- fast,
- safe,
- and efficient software.

And dividends will start to flow. Both in terms of time and money.

AWS Lambda, on the other hand, is a serverless platform that allows you to run your code without having to worry about the underlying infrastructure.

- It is a pay-as-you-go service, so you only pay for what you use.
- It is super popular among companies, especially in startups that want to build systems that can scale from 0 to 100x in a cost-efficient way, and quickly.
- This is the platform I have met over and over again working with startups, so I thought, "yes, let's use it".


Rust and AWS Lambda are the perfect match, because when you write your code in Rust, your program execution
time is super fast, which means your cloud bills will be super low.

And this is what we want, right?


## Our dataset

The dataset we will use for this example consists of historical taxi trip data from NYC taxis between 2017 and 2024. You can find the raw data
on [this public website](https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page).

> This is the kind of data that data engieeners at companies like Uber work with.

Every record in our dataset correponds to one taxi trip
```json
{
    // info available when the trip starts
    "tpep_pickup_datetime": "2024-01-30T01:17:07Z",
    "PUlocationID": 123,    // starting location id
    "DOlocationID": 456,    // ending location id

    // info available when the trip is completed
    "tpep_dropoff_datetime":"2024-01-30T01:24:06Z",
    "trip_distance":1.06,
    "fare_amount":11.58
}
```
This type of data can be used to solve many real world business problems like:

* Taxi trip duration prediction → as Uber does every time you request a aride.
* Taxi demand prediction for efficient taxi driver allocation → as DoorDash to optimize their fleet of delivery.
* ... you name it!


> **Wanna to solve a real world business problem with this data?**
>
> In the Real-World ML Tutorial you will learn to build, step by step, an ML system that predicts taxi demand in each area of NYC hour by hour.
>
> With this hands-on tutorial you will make the leap from ML model prototypes inside Jupyter notebooks to end-2-end ML systems, that generates live predictions that can be used to make business decisions.
>
> 👉 **[Click here to learn more](https://realworldmachinelearning.carrd.co/)**

Now, let's get back to our API!

## The tools we will use

We will need to install a few tools to get started:

- [The Rust compiler and Cargo package manager](https://www.rust-lang.org/tools/install)
- [Cargo lambda](https://www.cargo-lambda.info/guide/getting-started.html)
- [AWS CLI](https://aws.amazon.com/cli/)


## Bootstrapping the project

You can start a new Rust AWS Lambda project with the following command:

```bash
cargo lambda new rust-lambda-api --http
```

> We add the `--http` flag to create a Lambda function that can be invoked via HTTP, so you can
> run it behind an API Gateway and expose it to the rest of services of your company's infrastructure.

With this command, Cargo Lambda will create a new Rust project with a basic structure.
```
├── Cargo.toml
├── src
│ ├── http_handler.rs
│ └── main.rs
└── README.md
```

* The [Cargo.toml](./Cargo.toml) file is the equivalent of the pyproject.toml file in Python. It contains the dependencies and metadata of our project.
* The [src/main.rs](./src/main.rs) file is the entry point of our lambda function. No need to edit it.
* The [src/http_handler.rs](./src/http_handler.rs) file contains the logic of our API. This is the file we will start editing 👨🏻‍💻

## Developing the API



## Running the API locally



## Testing the API



## Building the API binary



## Deploying the API to AWS Lambda



## Invoking the API



## Wanna learn more Real World ML engineering with me?







## Wanna learn more Real World ML engineering with me?






