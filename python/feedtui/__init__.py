"""
feedtui - A configurable terminal dashboard for stocks, news, sports, and social feeds.

This package provides a Python interface to the feedtui Rust TUI application.

Usage:
    # As a CLI tool (after pip install)
    $ feedtui

    # As a Python module
    import feedtui
    feedtui.run()
"""

# Import the Rust module functions
from feedtui.feedtui import run, init_config, get_config_path, version

__version__ = version()
__all__ = ["run", "init_config", "get_config_path", "version", "__version__"]


def _cli_main():
    """CLI entry point for the feedtui command."""
    import sys
    import argparse

    parser = argparse.ArgumentParser(
        prog="feedtui",
        description="A configurable terminal dashboard for stocks, news, sports, and social feeds"
    )
    parser.add_argument(
        "-c", "--config",
        help="Path to config file (default: ~/.feedtui/config.toml)"
    )
    parser.add_argument(
        "-r", "--refresh",
        type=int,
        help="Refresh interval in seconds (overrides config)"
    )
    parser.add_argument(
        "-v", "--version",
        action="version",
        version=f"%(prog)s {__version__}"
    )

    subparsers = parser.add_subparsers(dest="command")

    # init subcommand
    init_parser = subparsers.add_parser("init", help="Initialize configuration with default settings")
    init_parser.add_argument(
        "-f", "--force",
        action="store_true",
        help="Force overwrite existing config"
    )

    # config subcommand
    subparsers.add_parser("config", help="Show current configuration path and status")

    args = parser.parse_args()

    if args.command == "init":
        try:
            config_path = init_config(force=args.force)
            print(f"Configuration created at: {config_path}")
        except FileExistsError as e:
            print(f"Error: {e}", file=sys.stderr)
            print("Use --force to overwrite.", file=sys.stderr)
            sys.exit(1)
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)
            sys.exit(1)
    elif args.command == "config":
        config_path = get_config_path()
        print(f"Config file: {config_path}")
        import os
        if os.path.exists(config_path):
            print("Status: Found")
        else:
            print("Status: Not found (run 'feedtui init' to create)")
    else:
        # Run the TUI
        try:
            run(config_path=args.config, refresh_interval=args.refresh)
        except KeyboardInterrupt:
            pass
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)
            sys.exit(1)


if __name__ == "__main__":
    _cli_main()
