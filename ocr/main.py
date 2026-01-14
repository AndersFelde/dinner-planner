import sys
import json
import os

os.environ["PADDLE_PDX_DISABLE_MODEL_SOURCE_CHECK"] = "1"

from paddleocr import PaddleOCR

ocr = PaddleOCR(
    lang="en",
    use_doc_orientation_classify=True,
    use_doc_unwarping=True,
    use_textline_orientation=True,
)


def main():
    if len(sys.argv) != 2:
        return False, json.dumps({"result": False, "error": "Usage: ocr.py <image_path>"})

    try:
        image_path = sys.argv[1]

        result = ocr.predict(input=image_path)

        out = []
        for r in result:
            out += r["rec_texts"]
        return json.dumps({"result": True, "text": out})
    except Exception as e:
        return json.dumps({"result": False, "error": f"Exception: {e}"})


if __name__ == "__main__":
    print(main())
