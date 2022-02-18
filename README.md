# enterprise add

Adding numbers together might seem simple, but how do you make it enterprise?

A bit of fun to make an isomorphic rust application in a mono repo that includes both frontend (webassembly) and backend (aws serverless).

## Getting started

Still a work in progress... but a `vs code/remote containers` combo should make life easier.
You'll need docker running so either **Docker for Mac/Windows** or the CLI if you're brave/need the
license and don't have it.

[colima](https://github.com/abiosoft/colima) does the business on mac. You'll need brew installed and:

``` bash
brew install colima
brew install docker
colima start --cpu 4 --memory 4
```

It needs to be 4gb otherwise it goes pop on `cargo make watch`. Talking of watch, sometimes it goes mad and
won't stop repeating. Don't know why...

Steps:

* make sure you have a aws profile for `add-number-tf` for **terraform**. Docker maps the .aws folder. Probably a better way for this...
* checkout repo
* open `code` in that folder
* hopefully the remote container magic will happen
* in the VS terminal...
* in `/workspaces/enterprise-add/lambda/add-numbers` do `cargo make packagelambda`
* same again in `/workspaces/enterprise-add/lambda/add-numbers`
* in `/workspaces/enterprise-add/terraform` do `cargo make deploy`
* phew! that should do the backend
* in `/workspaces/enterprise-add/frontend` do `cargo make watch` and `cargo make serve` in two separate windows
* open http://localhost:8000 in the browser

There's a lot going on here, but I can hopefully refine when i understand `cargo make` better.

**Note:** Safari does this weird thing where it starts throwing link errors when the page is loaded. Probably caching
as clearing all the web data sorts it.

## Requirements (not required now i've got remote containers working)

* rust (obvs)
* terraform
* cargo make
* cargo generate

## Remote containers

* remote containers work well because lambdas need to be build in docker because of macos thing
* tweaked Dockerfile as well to add in bits
* need a good way to add in aws credentials. going to mount .aws for the moment

## Random thoughts

* should `logic` expose serde, etc and the apps use them through that?
* should the whole thing be built through docker?

## Todo

[ ] call api from frontend
[ ] api gateway to sqs
[ ] sqs response through websockets
[ ] call async api from frontend
[ ] rust selenium anyone?
[ ] lambda logging
[ ] xray with spans'n'stuff
[ ] sqs message validation in api gateway
[ ] api keys?
[ ] better automation
[ ] s3 website for frontend