#!/usr/bin/env python
import os, sys

class File(object):
    def __init__(self, path, args):
        self.path = path
        self.args = args

    def render_completion(self):
        return "{} {}".format(self.path, " ".join(self.args))

    def render_execution(self):
        return "{} {}".format(self.path, " ".join(self.args))


class Directory(object):
    def __init__(self, path, args):
        self.path = path
        self.args = args

    def render_completion(self):
        """ should return a list of white-space separated options"""
        return " ".join(p for p in os.listdir(self.path))

    def render_execution(self):
        return "echo {} is a directory. tab-complete to choose subcommands".format(self.path)

def get_command(target_dir, args):
    """ 
    find the appropriate command to execute, given the arguments.

    match the args with directories within the target directory as they match,
    and pop those.

    once the current positional argument matches a file name, then that is the appropriate
    function to execute.
    """
    while len(args) > 0:
        arg = args[0]
        candidate_target = os.path.join(target_dir, arg)
        if os.path.isdir(candidate_target):
            target_dir = candidate_target
            args.pop(0)
            continue
        elif os.path.isfile(candidate_target):
            target_dir = candidate_target
            args.pop(0)
            break
        else:
            break

    if os.path.isfile(target_dir):
        return File(target_dir, args)
    elif os.path.isdir(target_dir):
        return Directory(target_dir, args)


def main(args=sys.argv[1:]):
    """ 
    args: a list of position arguments.

    * the first argument is always the path to the directory containing the scripts.
    """
    target_directory = args.pop(0)
    call_completion = (len(args) > 0 and "--complete" == args[-1])
    command = get_command(target_directory, args)
    if call_completion:
        print(command.render_completion())
    else:
        print(command.render_execution())

main()