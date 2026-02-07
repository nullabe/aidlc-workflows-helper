# Requirements Verification Questions

Please answer the following questions to help clarify the requirements for `aidlc-workflows-helper`.

## Question 1
Which platforms should the tool support for the interactive CLI prompts (the tool itself runs on the developer's machine)?

A) Kiro only (`.kiro/steering/` + `.kiro/aws-aidlc-rule-details/`)
B) Amazon Q Developer only (`.amazonq/rules/` + `.amazonq/aws-aidlc-rule-details/`)
C) Both Kiro and Amazon Q Developer (let the user choose)
D) All three: Kiro, Amazon Q Developer, and a generic "Other agent" option (custom path)
E) Other (please describe after [Answer]: tag below)

[Answer]: E - use the folder the user is prompted to - and also the script must change the core-workflow.md file to set the user choosed folder.

## Question 2
How should the tool fetch the aidlc-workflows release?

A) Always download the latest release zip from GitHub Releases API
B) Allow the user to specify a version (default to latest)
C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 3
Should the tool support running non-interactively (e.g., with CLI flags like `--platform kiro --gitignore yes`) for CI/scripting use?

A) Interactive only (always prompt the user)
B) Support both interactive and non-interactive (CLI flags override prompts)
C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
What should happen if the target rules folder already exists?

A) Overwrite silently (always replace)
B) Ask the user whether to overwrite or skip
C) Fail with an error message
D) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 5
For the GitHub Actions CI/CD, which targets should we build binaries for?

A) macOS (x86_64 + aarch64), Linux (x86_64), Windows (x86_64)
B) macOS (aarch64 only), Linux (x86_64), Windows (x86_64)
C) All of the above plus Linux aarch64
D) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 6
Should the binary be distributed as a single standalone executable (statically linked, no runtime dependencies)?

A) Yes, single static binary
B) Dynamically linked is fine
C) Other (please describe after [Answer]: tag below)

[Answer]: A
