# Sepa Direct debit


## Context

## Decision

## Consequences

## Status


## What is Direct Debit?

### Key point

1. Setting up mandates with customers' bank
2. Collecting payments against mandates
3. Staying update with webhooks

## Diggin through the integration

First of all we need to generate an access token for your application using GoCardless' dashboard.

### Customer

A customer could be a person or a company.
A mandate is an authorization from a customer to take payments from their bank account. Once a customer approves the mandate you can charge the customer with future API calls.

### How can I add a new customer?

You can use the Billing Request Flow or the Billing Request API



