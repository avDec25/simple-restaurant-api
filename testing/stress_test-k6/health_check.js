// k6 run --vus 10 --duration 30s health_check.js

// import { sleep } from 'k6';
import http from 'k6/http';

export const options = {
    // A number specifying the number of VUs to run concurrently.
    vus: 10,
    // A string specifying the total duration of the test run.
    duration: '30s',
};

export default function () {
    http.get(`http://localhost:8080/health_check`);
}
