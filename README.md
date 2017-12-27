# Wreath: Decorative Git Branches

Rewrite your git commit messages to make them a little more exciting :)

## Installation

1. [Install Cargo and Rust](http://doc.crates.io/#installing)

1. Clone this repository
    ```
    $ git clone git@github.com:GregOwen/Wreath.git
    ```

1. Build the binary
    ```
    $ cargo build
    ```

1. Add to path
    ```
    $ export PATH="$PATH:$(pwd)/target/debug"
    ```

## Usage

```
$ git checkout -b example-branch
Switched to a new branch 'example-branch'

$ git log --pretty=oneline
a83a7bf441d1d756077240491701799386db4060 (HEAD -> example-branch, master) Changed name to Wreath
697295cb6584be7f3adaa022140d5ae42ed73afd (examples) Added example messages
a161e05a913f7d469702eda899ea9d660efd3743 (strategies) Added CYCLE strategy
0058b5a619f4052824229f1640a1fa34c2473267 Cleaned up strategies
00f6a6b4915bf5ec165938cfbcd5137841a034c3 Cleaned up tests
0dc58588fa644a9a4dea4a363788999436798545 split("\n") --> lines()
270b0ed849a08b2637fa93d446b3771fd32bfb49 Handle case where we have more commits than message lines
bffc5c40316b276ea6ea576e33c092fda89f3d7f Test for default strategy
f2e92e4d09edc66e1d85a3a931cc1adb23c99574 Test strategy selection
3f5758f868c8c3b117c7759ebb9935f896b67a5a Select strategy based on env var
...

$ wreath messages/bad_horse.txt
Successfully rebased and updated refs/heads/example-branch.

$ git log --pretty=oneline
918517937f0dbe95dbebacf30430fedb069ef857 (HEAD -> example-branch) Bad Horse
9dc5ffb86667ac799cec8e2b80a77a668867f17f Bad Horse
204c7d8c50302548fbf6a94a289b44b06c37041d Bad Horse
f27a3179257a0451a16899c6ed58c8bf1112dab5 He's bad
99bc2274c915e6061b1298617a238a897c5170da He rides across the nation,
3425073c7bac305b770e51238e508df8a7ed828a The Thoroughbred of Sin
6a1cfa86368e426f379fd3b9c953f531f4f2992b He got the application that you just sent in!
fe83c569ab885a8e74b2f9076a8ec2d002fe00cc It needs evaluation, so let the games begin
12ad10fc7ac42eaef730b0ec1bf5cd48bd36ab6b A heinous crime, a show of force
24b6ca732b18347b203feda08d515f137a272fe0 (A murder would be nice of course)
...
```

### Using a Different Strategy

Let's say your file of replacement commit messages only has `m` lines, but you have `n > m` commits
in your history. By default, Wreath will replace the `m` most recent commit messages and leave the
remaining messages untouched. You can change this behavior using the `WREATH_STRATEGY` environment
variable. For example, the following will loop through your list of replacement commit messages over
and over again until all commits have been updated:

```
$ WREATH_STRATEGY=CYCLE wreath /path/to/new/message/file.txt
```

For a list of the available strategies, see [the strategies file](src/strategies.rs).
