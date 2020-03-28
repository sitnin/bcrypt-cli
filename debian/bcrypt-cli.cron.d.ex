#
# Regular cron jobs for the bcrypt-cli package
#
0 4	* * *	root	[ -x /usr/bin/bcrypt-cli_maintenance ] && /usr/bin/bcrypt-cli_maintenance
