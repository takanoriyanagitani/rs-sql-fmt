#!/bin/sh

which wazero | fgrep -q wazero || exec sh -c 'echo wazero missing.; exit 1'
which bat | fgrep -q bat || exec sh -c 'echo bat missing.; exit 1'

echo '
	select * from table1
	where
		id = 1
		and active = true
' |
	wazero \
		run \
		./rs-sql-fmt.wasm |
	bat --language=sql
