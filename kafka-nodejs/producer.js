// producer.js

const { Kafka } = require("kafkajs");

const kafka = new Kafka({
  clientId: "my-app",
  brokers: ["localhost:9092"],
});

const producer = kafka.producer();

(async () => {
  await producer.connect();
  await producer.send({
    topic: "test-topic",
    messages: [{ value: "Hello KafkaJS  the request is is user!6f1e92d2-c166-467c-b640-776b71628cd7 " }],
  });

  await producer.disconnect();
})();
