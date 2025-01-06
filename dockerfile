# ベースイメージの指定
FROM rastasheep/ubuntu-sshd

# メンテナー情報（任意）
LABEL maintainer="seg4desk@gmail.com"


# パッケージのインストール
RUN apt-get update && export DEBIAN_FRONTEND="noninteractive apt-get upgrade -y" && apt-get -y install iputils-ping && apt-get -y install net-tools && apt-get -y install locales && apt-get -y install git && apt-get -y install build-essential && apt-get -y install pkg-config vim curl postgresql postgresql-contrib libssl-dev nginx && sed -i -e 's@/var/www:/usr/sbin/nologin@/var/www:/bin/bash@' /etc/passwd && sed -i -e 's@127.0.0.1/32            md5@127.0.0.1/32            trust@g' /etc/postgresql/10/main/pg_hba.conf && sed -i -e 's@/var/www/html@/var/tmp/html@g' /etc/nginx/sites-available/default && mv /var/www/html /var/tmp/ && echo "set encoding=utf-8" >> ~/.vimrc && cp -p ~/.vimrc /var/www/ && chown -R www-data.www-data /var/www && su - www-data -c "cd ~/ && curl https://sh.rustup.rs -sSf | sh -s -- -y " && echo ". ~/.vimrc" >> /var/www/.profile && su - www-data -c "cd ~/ && git clone https://github.com/segmentation4lt/fasbone" && eval "sed 's/<FQDN>/$(hostname)/g' /var/www/fasbone/nginx_config.txt" >> /etc/nginx/sites-available/fasbone && cd /etc/nginx/sites-enabled && ln -sv /etc/nginx/sites-available/fasbone ./ && locale-gen ja_JP.UTF-8 && update-locale LC_ALL=ja_JP.UTF-8 LANG=ja_JP.UTF-8 && service postgresql start && su - postgres -c "psql -f /var/www/fasbone/docker_create_database.txt" && su - postgres -c "psql -U fasbone -h localhost -p5432 fasbone -f /var/www/fasbone/create_sql.txt" && sed  -i -e "10 s/;;/export RUST_COREDIR=\"\$HOME\/devel\/Rust\"\n        export RUST_LOG=\"Debug\"\n        export PG_CONNECT_HOST=\"localhost\"\n        export PG_CONNECT_USER=\"fasbone\"\n        export PG_CONNECT_PASS=\"fasbone\"\n        export PG_CONNECT_DATABASE=\"fasbone\"\n        export PG_CONNECT_PORT=\"5432\"\n        export ENCRYPT_KEY=\"%p_kMvkZfB5xMTYBayqGf_y9,h#giLMk\"\n        export MAIL_FROM=\"Localhost（ローカルホスト） <no-reply@localhost.localdomain>\"\n        export SMTP_AUTH_USER=\"@gmail.com\"\n        export SMTP_AUTH_KEY=\"\"\n        ;;/" /var/www/.cargo/env 



# コンテナ起動時のコマンド指定
CMD ["/bin/bash"]

