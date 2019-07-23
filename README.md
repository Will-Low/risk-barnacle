# Risk Barnacle

Risk Barnacle is a tool to help quantitatively assess risk. It converts different risk scenarios into a probability distribution of monetary losses at different amounts.

## How Does It Work?

Risk Barnacle determines your annual risk exposure by simulating a year in the life of your company thousands of times. It looks for years when a monetary loss occurs, determines the probability of the loss happening in a year, and calculates the size of the loss.

## Why Use Risk Barnacle?

Risk, at least in the information security space, is usually evaluated using “ordinal scales,” systems of measure such as “low, medium, high” or “CVE 1-10.” There is meaning within these scales, such as “a medium priority vulnerability is more important than a low priority vulnerability,” though all meaning breaks down when trying to compare rankings outside of the context of the scale. For example, the total risk of having 10 issues of vulnerability level “5” does not equal the total risk of having 5 issues of vulnerability level “10".

We want to be able to evaluate our security risks on a “ratio scale,” meaning that we _could_ compare the risk of 10 instances one type of vulnerability to 5 of another type, while preserving meaning. This would require us to have “risk units” that we’d  use to compare to one risk to another. The most straightforward unit that we could do this in is in dollars, or another currency of our choice, which is what Risk Barnacle does.

## How to Use It
