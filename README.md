# enterprise add

Adding numbers together might seem simple, but how do you make it enterprise?


## Requirements

* rust (obvs)
* terraform
* cargo make
* cargo generate

## Remote containers

* lambdas need to be build in docker because of macos thing
* calling docker from makefile task has probs when linking logic create
* oo :) let's see if it works with vs code remote containers
* then i can simplify the lambda build (no docker ness.)
* docker needs to be root or cargo fails (CARGO_HOME has a weird setting)
* tweaked Dockerfile as well to add in bits
* need a good way to add in aws credentials. going to mount .aws for the moment

## Random thoughts

* should `logic` expose serde, etc and the apps use them through that?
* should the whole thing be built through docker?
