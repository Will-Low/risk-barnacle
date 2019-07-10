# Risk Barnacle

Risk Barnacle is a tool to help quantitatively assess risk. It converts different risk scenarios into a probability distribution of monetary losses at different amounts.

## Why Is This Needed?

Risk, at least in the information security space, is most frequently evaluated using “ordinal scales,” such as “low, medium, high” or “CVE 1-10.” There is meaning within these scales, such as “a medium priority vulnerability is more important than a low priority vulnerability,” though all meaning breaks down when trying to operate on these rankings outside of the context of the scale. For example, the risk of having 10 issues of vulnerability level “5” does not equal the risk of having 5 issues of vulnerability level “10”—this is nonsensical. 

We want to be able to evaluate our security risks on a “ratio scale,” meaning that we _could_ compare the risk of 10 instances one type of vulnerability to 5 of another type, all while preserving meaning. This essentially requires us to have “risk units” that we’ll compare to one risk to another. The most straightforward unit that we could do this in is in dollars, or another currency of our choice.

## How Does It Work?

Risk Barnacle uses probability distributions of different scenarios happening in order to develop a probability distribution of monetary loss per unit time.

Here are a few concepts to introduce:

**Loss Event** - refers to a scenario that results in a monetary loss of some sort

**Event** - an event of . Its unit is: number of initiating events/unit time

**Condition** - indicates a control or condition that “filters” the probability that an event results in a loss event. Its unit is: % ineffectiveness of the control/condition.

**Magnitude** - in the event of a loss, the number of units lost/damaged/etc. Its unit is: # units affected by the loss event 

**Cost** - a cost per affected-by-loss-event unit
