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
                ruby     1.755k i/100ms
                rust    41.775k i/100ms
Calculating -------------------------------------
                ruby     17.612k (± 0.5%) i/s   (56.78 μs/i) -     89.505k in   5.082041s
                rust    411.681k (± 2.0%) i/s    (2.43 μs/i) -      2.089M in   5.075727s

Comparison:
                rust:   411680.8 i/s
                ruby:    17612.4 i/s - 23.37x  slower
```

## Usage

```ruby
require 'faster_cosine'

vector1 = [1.8453342946, -0.3523491036, 0.2349104862]
vector2 = [1.2463771752, -0.2146283944, 0.6624209229]
FasterCosine.distance(vector1, vector2)
```
