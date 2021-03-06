import csv
import random
import time
import uuid

TRANSACTIONS_FILE = "src/pending_transactions.txt"
MAX_MONEY_TRANSFER = 10000

def generate(amount_transactions, amount_users):
  user_ids = [str(uuid.uuid4()) for _ in range(amount_users)]
  curr_timestamp = int(time.time())
    
  with open(TRANSACTIONS_FILE, "w") as f:
    transactions_file = csv.writer(f)
    for i in range(1, amount_transactions + 1):
      transactions_file.writerow(
        [i, random.choice(user_ids), curr_timestamp, 
         random.choice(["cash_in", "cash_out"]), random.random() * MAX_MONEY_TRANSFER]
      )
      curr_timestamp += random.randint(0, 30000)
      
  
generate(20, 5)
