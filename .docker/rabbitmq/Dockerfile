ARG VERSION=3.13.0

FROM rabbitmq:${VERSION}-management-alpine

RUN apk update && apk add curl
RUN curl -L https://github.com/rabbitmq/rabbitmq-delayed-message-exchange/releases/download/v3.13.0/rabbitmq_delayed_message_exchange-3.13.0.ez > $RABBITMQ_HOME/plugins/rabbitmq_delayed_message_exchange-3.13.0.ez
RUN chown rabbitmq:rabbitmq /plugins/rabbitmq_delayed_message_exchange-3.13.0.ez
RUN rabbitmq-plugins enable rabbitmq_delayed_message_exchange