ARG RABBITMQ_VERSION=3.13.0

FROM rabbitmq:${RABBITMQ_VERSION}-management-alpine

RUN apk update && apk add curl
RUN curl -L https://github.com/rabbitmq/rabbitmq-delayed-message-exchange/releases/download/v$RABBITMQ_VERSION/rabbitmq_delayed_message_exchange-$RABBITMQ_VERSION.ez > $RABBITMQ_HOME/plugins/rabbitmq_delayed_message_exchange-$RABBITMQ_VERSION.ez
RUN chown rabbitmq:rabbitmq /plugins/rabbitmq_delayed_message_exchange-$RABBITMQ_VERSION.ez
RUN rabbitmq-plugins enable rabbitmq_delayed_message_exchange