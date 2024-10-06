# frozen_string_literal: true

require "test_helper"
require "time"

class TestSysGvl < Minitest::Test
  def setup
    @tmpdir = Dir.mktmpdir("watchcat")
  end

  def teardown
    FileUtils.remove_entry_secure(@tmpdir)
  end

  def test_slow_func
    t = Thread.new do
      assert_equal "Hello, Tanaka!", SysGvl.slow_func("Tanaka")
    end

    create_file
    t.join
  end

  def test_not_lock_slow_func
    t = Thread.new do
      assert_equal "Hello, Tanaka!", SysGvl.not_lock_slow_func("Tanaka")
    end

    create_file
    t.join
  end

  def test_instance_slow_func
    t = Thread.new do
      assert_equal "Hello, Tanaka!", SysGvlCls.new.slow_func("Tanaka")
    end

    create_file
    t.join
  end

  private

  def create_file
    puts "will create a file #{Time.now}"
    FileUtils.touch(File.join(@tmpdir, "a.txt"))
    puts "created a file #{Time.now}"
  end
end
