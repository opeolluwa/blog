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
});;

const consumer = kafka.consumer({
  groupId: "test-topic",
  fromBeginning: true,
});

(async () => {
  await consumer.connect().then(() => {
    console.log("Client connected");
  });
  await consumer.subscribe({
    topic: "test-topic",
    fromBeginning: true,
  });

  await consumer.run({
    eachMessage: async ({ topic, partition, message }) => {
      console.log({
        value: message.value.toString(),
      });
    },
  });
})();
