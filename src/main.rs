import os
import asyncio
from flask import Flask, jsonify
from flask_cors import CORS
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

# Initialize Flask application
app = Flask(__name__)

# Enable CORS for all domains
CORS(app)

# Get port from environment variable or use 3030 as default
port = int(os.getenv('PORT', 3030))

# Define a route for "/products" that returns a list of products
@app.route('/products', methods=['GET'])
async def get_products():
    # List of products
    products = [
        {"id": 1, "name": "Dog Food", "price": 19.99},
        {"id": 2, "name": "Cat Food", "price": 34.99},
        {"id": 3, "name": "Bird Seeds", "price": 10.99}
    ]
    # Simulate asynchronous behavior with asyncio.sleep
    await asyncio.sleep(0.1)  # Simulates async processing
    return jsonify(products)

# Run the Flask app asynchronously
async def run_app():
    loop = asyncio.get_event_loop()
    loop.run_in_executor(None, app.run, '0.0.0.0', port)

# Main function to start the server
if __name__ == '__main__':
    asyncio.run(run_app())
