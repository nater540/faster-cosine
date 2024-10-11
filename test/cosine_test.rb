require 'test/unit'
require_relative '../lib/faster_cosine'

class CosineTest < Test::Unit::TestCase
  def test_similarity
    vector1 = [1.687813487, -1.723201293, 0.289894724]
    vector2 = [1.765887184, -1.896733895, 0.230743419]
    assert { FasterCosine.similarity(vector1, vector2) == 0.9992081252727524 }
  end
end
