# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"

Minitest::TestTask.create

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("sys_gvl.gemspec")

RbSys::ExtensionTask.new("sys_gvl", GEMSPEC) do |ext|
  ext.lib_dir = "lib/sys_gvl"
end

task default: %i[compile test]
