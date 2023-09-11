import os
from dotenv import load_dotenv
from cassandra.auth import PlainTextAuthProvider
from cassandra.cluster import Cluster
from langchain.embeddings import OpenAIEmbeddings
from langchain.vectorstores.cassandra import Cassandra
from datasets import load_dataset

# Load environment variables from .env file
load_dotenv()

# Access your variables
ASTRA_DB_APPLICATION_TOKEN = os.getenv("ASTRA_DB_APPLICATION_TOKEN")
ASTRA_DB_CLIENT_ID = os.getenv("ASTRA_DB_CLIENT_ID")
ASTRA_DB_CLIENT_SECRET = os.getenv("ASTRA_DB_CLIENT_SECRET")
ASTRA_DB_KEYSPACE = os.getenv("ASTRA_DB_KEYSPACE")
ASTRA_DB_SECURE_BUNDLE_PATH = os.getenv("ASTRA_DB_SECURE_BUNDLE_PATH")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

cloud_config = {
    'secure_connect_bundle': ASTRA_DB_SECURE_BUNDLE_PATH
}

auth_provider = PlainTextAuthProvider(ASTRA_DB_CLIENT_ID, ASTRA_DB_CLIENT_SECRET)
cluster = Cluster(cloud=cloud_config, auth_provider=auth_provider)
astraSession = cluster.connect()

# Create an OpenAI Embeddings instance
myEmbeddings = OpenAIEmbeddings(openai_api_key=OPENAI_API_KEY)

# Set the table name
table_name = "qa_min_demo"

# Create a Cassandra instance
myCassandraVStore = Cassandra(
    session=astraSession,
    keyspace=ASTRA_DB_KEYSPACE,
    embedding=myEmbeddings,
    table_name=table_name
)

# Continue with your code as before
print("Loading dataset...")
myDataset = load_dataset("Biddls/Onion_News", split="train")
headlines = myDataset["text"][:50]

print("Indexing dataset...")
myCassandraVStore.add_texts(headlines)

print("Inserted %i headlines.\n" % len(headlines))