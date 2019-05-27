[![Build Status](https://travis-ci.com/icedrift/tusk.svg?branch=master)](https://travis-ci.com/icedrift/tusk)
[![codecov](https://codecov.io/gh/icedrift/tusk/branch/master/graph/badge.svg)](https://codecov.io/gh/icedrift/tusk)

# Tusk


## Installation

## Terminology
|Word|Meaning|
|----|-------|
|Tag|A string associated with a reading|
|Dimension|A key-value pair associated with a reading|
|Metadata|A combination of tags and dimensions associated with a reading|
|Data Point|A combination of a name and a value, an example would be temperature and 24|
|Reading|A combination of metadata and datapoints with a timestamp|

## Structure
The Tusk agent consists of the main agent and a selection of plugins.

### Inputs
Input plugins provide a series of readings from a source.

### Outputs
Input plugins send a series of readings to a destination.

### Pipelines
Pipelines are a series of processing steps connecting inputs and outputs that can transform or aggregate the data.

## Configuration
### Configure Tusk

### Configure the plugins

#### Inputs

#### Outputs

#### Pipelines
