# Linear Dynamic Scheduler

Linear Dynamic Scheduler (LDS) is a Rust-based system for scheduling and executing background tasks.

It separates job submission from execution by placing work into a queue and processing it asynchronously through workers.

The goal is to explore how real backend systems handle reliability, concurrency, and distributed execution.

## Why it exists

Modern applications often need to run work that should not block a request, such as sending emails, processing files, or generating reports.

LDS is built to understand how these systems are designed and coordinated.

## Core idea

* Jobs are submitted and stored in a queue
* A scheduler decides execution order
* Workers process jobs independently
* Results are tracked until completion

## Components

* Scheduler: decides what runs and when
* Queue: stores pending jobs
* Workers: execute jobs in background
* Jobs: units of work submitted by users

## Goal

Build a reliable system that can process background tasks safely, even under failures, delays, and multiple workers.
