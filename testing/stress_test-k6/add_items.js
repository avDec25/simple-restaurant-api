// k6 run add_items.js --console-output=console_out.txt

import http from 'k6/http';
import { dishes } from './menu_items.js';

function generate_random_order(order_items) {
    var order = []
    for (let i = 0; i < order_items; i++) {
        order.push(dishes[Math.floor(Math.random() * dishes.length)])
      } 
    return order;
}

export const options = {
    vus: 10,
    duration: '30s',
};


export default function () {
    const params = {
        headers: {
            'content-type': 'application/json'
        }
    };

    const TOTAL_TABLES = 200;
    const MAX_TABLE_ITEMS = 5;
    const table_number = Math.floor(Math.random() * TOTAL_TABLES) + 1;

    const payload = JSON.stringify({
        "table_number": table_number,
        "items_names" : generate_random_order(Math.floor(Math.random() * MAX_TABLE_ITEMS) + 1)
    })

    http.post(`http://localhost:8080/table/add_items`, payload, params);
}
