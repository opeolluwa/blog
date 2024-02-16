//producer.js
const { Kafka } = require("kafkajs");

const kafka = new Kafka({
  clientId: "my-app",
  brokers: ["localhost:9092"],
  ssl: true,
  sasl: {
    username: "username",
    password: "password",
    mechanism: "plain",
  },
});

const producer = kafka.producer();

(async () => {
  await producer.connect();
  await producer.send({
    topic: "test-topic",
    messages: [{ value: "Hello KafkaJS user!" }],
  });

  await producer.disconnect();
})();
