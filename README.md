# Tome

Take a bunch of scripts, in directories of any depth:

```
root/
  go-to-workspace
  android/
    debug
  python/
    start-virtualenv
  work/
    start-my-server
    my-team/
      provision-my-service
```

And convert that into a single command that can run those into commands:

```
$ my-command go-to-workspace (executes root/go-to-workspace)
$ my-command python start-virtualenv (executes root/python/start-virtualenv)
$ my-command work my-team provision-my-service (executes root/work/my-team/provision-my-service)
```

# Building Tome

See bin/build-release-package