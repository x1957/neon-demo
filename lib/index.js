var addon = require('../native');

addon.threadCountCb("thread_count: ", (result) => {
    console.log(result);
});

console.log(addon.threadCount());