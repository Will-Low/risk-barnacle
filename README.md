<p align="center">
  <img src="https://user-images.githubusercontent.com/26700668/62333746-b831b100-b492-11e9-97a0-654571cac535.png" width="40%" align="">
</p>
Risk Barnacle is a tool to help quantitatively assess risk. It converts different risk scenarios into a probability distribution of monetary losses at different amounts.

## Why Use Risk Barnacle?

Risk, at least in the information security space, is usually evaluated using “ordinal scales,” systems of measure such as “low, medium, high” or “CVE 1-10.” There is meaning within these scales, such as “a medium priority vulnerability is more important than a low priority vulnerability,” though all meaning breaks down when trying to compare rankings outside of the context of the scale. For example, the total risk of having 10 issues of vulnerability level “5” does not equal the total risk of having 5 issues of vulnerability level “10".

We want to be able to evaluate our security risks on a “ratio scale,” meaning that we _could_ compare the risk of 10 instances one type of vulnerability to 5 of another type, while preserving meaning. This would require us to have “risk units” that we’d  use to compare to one risk to another. The most straightforward unit that we could do this in is in dollars, or another currency of our choice, which is what Risk Barnacle does.

## How Does It Work?

Risk Barnacle determines your annual risk exposure by simulating a year in the life of your company thousands of times. It looks for years when a monetary loss occurs, determines the probability of the loss happening in a year, and calculates the size of the loss.

Risk Barnacle's primary functionality consists of the following parts:

1. Three "library" files: `events.yaml`, `conditions.yaml`, and `costs.yaml`, where lists of events, conditions, and costs are stored, respectively. An "event" refers to the initial thing that could ultimately result in a financial loss. Events can always be thought of as something that could occur a certain number of times a year. For example, the number of times a user receives a malicious email is an event. A "condition" is a particular state or control that may prevent the event from resulting in a financial loss. For example, antivirus products, which could stop a malware infection, would be a good condition include. Lastly, a "cost" is a way of representing a per-unit expense. If you were developing a scenario to represent a malware stealing you sensitive data, you may have a cost that grows with the amount of sensitive data lost, such as settlement fees in the event of a class-action suit.
2. A series of "play" files or "plays," which each consist of one event, one or more conditions, a "magnitude" (the number of units lost, in the event of the controls failing), and one or more per-unit costs.

Risk Barnacle's risk engine takes the supplied value ranges, supplied in the events, conditions, and costs files, and plugs them in to the play files. Using these values, it then calculates the annual likelihood of each play resulting in a financial loss: the event occurs, the conditions fail to stop the threat, and the resulting size of the loss.

## How to Use It
Write a new play file (see the demo file at `plays/demo_play.yaml` for an example) and add the referenced events, conditions, and costs to their respective files.

Run Risk Barnacle with `barnacle` command (release binary located at `target/release/barnacle`), which will run 100,000 iterations by default. This may be changed with the `-i` or `--iterations` flags. It'll print results and save them as a CSV file in the `output` directory. By default, it titles each run with the current UTC time, though this may be overridden with the `-o` or `--output` flags.

## How to Build It
1. Install the Rust development environment (Rustup), using the instructions here: https://www.rust-lang.org/tools/install
2. Navigate to the `barnacle` folder
3. Build Risk Barnacle using `cargo build` for a slower, debug version or `cargo build --release` for the faster, optimized version (recommended).
4. Run Risk Barnacle by navigating to the `barnacle` directory and running `./target/release/barnacle`.
