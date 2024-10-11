# Faster Cosine

```shell
bundle
rake compile
rake test
rake bench
```

```
ruby 3.3.1 (2024-04-23 revision c56cd86388)
Warming up --------------------------------------
                ruby     1.747k i/100ms
                rust    40.642k i/100ms
Calculating -------------------------------------
                ruby     17.351k (± 4.3%) i/s   (57.63 μs/i) -     87.350k in   5.047420s
                rust    405.764k (± 0.8%) i/s    (2.46 μs/i) -      2.032M in   5.008449s

Comparison:
                rust:   405763.7 i/s
                ruby:    17350.9 i/s - 23.39x  slower
```

## Usage

```ruby
require 'faster_cosine'

vector1 = [1.8453342946, -0.3523491036, 0.2349104862]
vector2 = [1.2463771752, -0.2146283944, 0.6624209229]
FasterCosine.similarity(vector1, vector2)
```
