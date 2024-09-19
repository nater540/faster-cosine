require_relative 'lib/faster_cosine/version'

Gem::Specification.new do |spec|
  spec.name = 'faster_cosine'
  spec.version = FasterCosine::VERSION
  spec.summary = 'Faster cosine distance'
  spec.authors = ['Nate Strandberg']
  spec.files = Dir['*.{md,txt}", "{lib}/**/*']
  spec.requirements = ['Rust >= 1.81']
  spec.require_path = 'lib'
  spec.required_ruby_version = '>= 3.2'

  spec.add_development_dependency 'rake-compiler', '~> 1.2'
  spec.add_development_dependency 'rb_sys', '~> 0.9'
  spec.add_development_dependency 'test-unit', '~> 3.6'
  spec.add_development_dependency 'benchmark-ips', '~> 2.14'
end
