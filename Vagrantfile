# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "rust_rabbitmq_example"

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.provider "docker" do |d|
      d.image = "allansimon/docker-dev-rust"
      d.name = "#{PROJECT}_dev"
      d.has_ssh = true
      d.env = {
        "HOST_USER_UID" => Process.euid,
      }
    end

    # libssl-dev is required for compilation with amqp
    app.vm.provision "installs", "type": "shell" do |installs|
      installs.inline = "
        sudo apt-get update
        sudo apt-get install libssl-dev -y
      "
    end
    app.ssh.username = "vagrant"
  end

  config.vm.define "db" do |db|
    db.vm.provider "docker" do |d|
      d.image = "rabbitmq"
      d.name = "#{PROJECT}_queue"
    end
  end
end
