require 'test/unit'
require_relative '../lib/faster_cosine'

class CosineTest < Test::Unit::TestCase
  def test_distance
    assert { FasterCosine.distance([1,1,0], [1,0,0]) == 0.29289321881345254 }
  end
end
