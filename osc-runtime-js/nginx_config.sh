#!/bin/sh

# Copyright 2018 The OpenSDS Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

LISTEN_PORT=${LISTEN_PORT:-8080}
cat > /etc/nginx/conf.d/default.conf <<EOF
    server {
        listen ${LISTEN_PORT};
        listen [::]:${LISTEN_PORT};
        root /var/www/html;
        index index.html;
        server_name _;
        location /v1alpha/ {
            proxy_pass http://127.0.0.1:6106/v1alpha/;
        }
    }
EOF

sudo rm -R /var/www/html/
sudo mkdir /var/www/html/ -p
sudo cp -R ./* /var/www/html/

# print some log to stdin
echo "Service Start Time $(date)"
echo "Configuration /etc/nginx/conf.d/default.conf"
cat /etc/nginx/conf.d/default.conf

# start nginx service
service nginx restart
