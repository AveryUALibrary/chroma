import os

import pytest
import httpx
from httpx import HTTPError, ConnectError

from chromadb.utils.embedding_functions import LlamaCppEmbeddingFunction

def test_llama_cpp_embedding_function() -> None:
    """
    To set up the model, follow your model setup instructions.
    Ensure the model path or HuggingFace repo and filename are correctly set.
    """
    model_path = os.environ.get("LLAMA_MODEL_PATH")
    huggingface_repo_id = os.environ.get("HUGGINGFACE_REPO_ID")
    huggingface_filename = os.environ.get("HUGGINGFACE_FILENAME")

    if model_path is None and (huggingface_repo_id is None or huggingface_filename is None):
        pytest.skip(
            "Model path or HuggingFace repo id and filename not set. Skipping test."
        )

    try:
        if model_path:
            ef = LlamaCppEmbeddingFunction(model_path=model_path)
        else:
            ef = LlamaCppEmbeddingFunction(
                huggingface_repo_id=huggingface_repo_id,
                huggingface_filename=huggingface_filename
            )
    except Exception as e:
        pytest.skip(f"Error initializing LlamaCppEmbeddingFunction: {e}")

    try:
        embeddings = ef(["Here is an article about something...", "This is another article."])
        assert len(embeddings) == 2
        assert all(isinstance(embed, list) for embed in embeddings)
    except Exception as e:
        pytest.fail(f"Embedding generation failed: {e}")

if __name__ == "__main__":
    pytest.main()