# Contributing

Welcome to our repository! This document covers...
* Standard development workflow
* Code style

## Development practice

Here's the flow for code under regular conditions:

*Feature branch created* (by Developer)  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &darr;  
*Feature implemented and tested on branch*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &darr;  
*PR created*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &darr;  
*PR reviewed* (by Code Owner)  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &darr; &uarr;  
*PR tested and fixed if needed* (by Developer and Code Owner)  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &darr;  
*PR merged to main*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &darr;  
*Delete feature branch*

Ongoing projects may have persistent branches that are synced with main through PRs, but this
should be limited.

Squash merges are not recommended, but this may change in the future.

Code owners also have the power to force merge PRs, which should be used in limited scenarios.  
&nbsp;&nbsp;&rdsh; *Why would you do that?* See [Appendix A](#appendix-a)

## Code style

For the most part, just do whatever you think is good. Good variables are a must, and so
are documentation comments. Follow standard Rust conventions where applicable (i.e. case for
variables, functions, structs).

## Appendix
### Appendix A

There are two reasons to allow force merging PRs:

1. Competition environment
When competition rolls around, things need to be done quickly, on the time scale of hours or minutes. In
such situations, waiting for another code owner to perform a review is impractical, especially if it
is required through the github system.

2. Formalism fatigue
Although review is generally desired whenever possible, requiring the formalism can create detrimental friction. For
example, if a code owner watches the testing and reviews the changes another makes in person, there is unnecessary friction
in the reviewer going through the regular process. Such friction can slow things down (adding merge conflicts), and create
perverse incentives (such as cramming too much into one PR).

