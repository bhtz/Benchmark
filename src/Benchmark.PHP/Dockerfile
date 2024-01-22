FROM php:8.3.1-apache

# PHP
RUN apt-get update && apt-get upgrade
RUN apt-get install -y zlib1g-dev libwebp-dev libpng-dev && docker-php-ext-install gd
RUN apt-get install libzip-dev -y && docker-php-ext-install zip

ENV COMPOSER_ALLOW_SUPERUSER=1
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer

COPY ./composer.json ./composer.lock /var/www/html/

RUN composer install

COPY ./000-default.conf /etc/apache2/sites-available/000-default.conf

COPY ./ /var/www/html

RUN a2enmod rewrite
RUN service apache2 restart