<div align="center">
  <img src="https://github.com/merrickliu888/cody/blob/main/public/cody_icon.png" alt="cody-icon" width="75">  
  <h1>Cody</h1>
  <h3>AI Native Unix Shell</h3>
</div>

## About

Cody is an AI Native Unix shell that can translate natural language into shell commands on top of supporting some basic shell functionality. AI capabilities are availabale offline and online as Cody can either deploy a local LLM or use a hosted LLM.

## Implementation

Cody is built using Rust and uses [Ollama](https://github.com/ollama/ollama) to run an LLM (Llama3.1 8b) locally, and uses Cohere as the hosted LLM.

## Setup

Download the [Cody binary](https://github.com/merrickliu888/cody/blob/main/dist/cody) then:

To setup local LLM

1. Install Ollama: `curl -fsSL https://ollama.com/install.sh | sh`
2. Create a model using the given `Modelfile` (`ollama create mymodel -f ./Modelfile`)

To setup online

1. Get a [Cohere API key](https://dashboard.cohere.com/api-keys) and put it in a `.env` file.
2. Place the `.env` file in the same directory as the `Cody` binary.

## Use

To start Cody:

- `./cody` - Cody defaults to using the hosted LLM.

- `./cody online`- Initialize Cody with the hosted LLM.

- `./cody offline` - Initialize Cody with the local LLM.

For the most part you can use regular shell commands like:

- `ls > output.txt`
- `cat foo.txt | grep xyz`
- `cd ..`
- `x="5"`
- `exit`

To use AI to convert natural language to shell commands, prefix the prompt with `cody` like this:

```
$ cody list all the files in this directory
Generated Shell Command: ls
foo.txt  foo2.md  foo3.rs
```
