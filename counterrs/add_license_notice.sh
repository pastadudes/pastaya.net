#!/usr/bin/env bash

# you should run this as a git hook NOT cronjob
LICENSE_HEADER=' /**
 *
 * @licstart  The following is the entire license notice for the
 *  JavaScript code in this page.
 *
 * Copyright (C) 2025 pastaya
 *
 *
 * The JavaScript code in this page is free software: you can
 * redistribute it and/or modify it under the terms of the GNU
 * General Public License (GNU GPL) as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option)
 * any later version.  The code is distributed WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU GPL for more details.
 *
 * As additional permission under GNU GPL version 3 section 7, you
 * may distribute non-source (e.g., minimized or compacted) forms of
 * that code without the copy of the GNU GPL normally required by
 * section 4, provided you include this license notice and a URL
 * through which recipients can access the Corresponding Source.
 *
 * @licend  The above is the entire license notice
 * for the JavaScript code in this page.
 *
 */
 '

# loop through all .js files in the pkg directory
for file in pkg/*.js; do
    # check if the license header is already in the file
    if grep -q "Copyright (C) 2025 pastaya" "$file"; then
        echo "license header already exists in $file. skipping."
    else
        # prepend the license header to each file
        echo -e "$LICENSE_HEADER\n$(cat "$file")" > "$file"
        echo "added license header to $file"
    fi
done
