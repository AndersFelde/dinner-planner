import json
import os
import sys
from multiprocessing import cpu_count

import numpy as np
from PIL import Image, ImageEnhance, ImageOps

os.environ["PADDLE_PDX_DISABLE_MODEL_SOURCE_CHECK"] = "1"

from paddleocr import PaddleOCR


def get_image(path: str):
    img = Image.open(path)
    img = ImageOps.exif_transpose(img)
    
    max_side = 1500
    w, h = img.size
    scale = min(max_side / w, max_side / h, 1.0)
    new_w, new_h = int(w * scale), int(h * scale)
    img_resized = img.resize((new_w, new_h), Image.LANCZOS)
    
    return np.array(img_resized)



def main():
    if len(sys.argv) != 2:
        return json.dumps({"result": False, "error": "Usage: ocr.py <image_path>"})

    try:
        image_path = sys.argv[1]
        image = get_image(image_path)

        ocr = PaddleOCR(
            lang="en",
            text_detection_model_name="PP-OCRv5_mobile_det",
            text_recognition_model_name="en_PP-OCRv5_mobile_rec",
            cpu_threads=cpu_count(),
            use_doc_orientation_classify=True,
            use_doc_unwarping=True,
            use_textline_orientation=True,
        )
        result = ocr.predict(image)

        lines = []

        for r in result:
            items = list(zip(r["rec_boxes"], r["rec_texts"]))

            # Sort top to bottom
            items.sort(key=lambda x: x[0][1])

            current_line = []
            current_y = None
            Y_THRESHOLD = 20  # tweak if needed

            for box, text in items:
                y = box[1]  # top y

                if current_y is None or abs(y - current_y) <= Y_THRESHOLD:
                    current_line.append((box[0], text))  # keep x for sorting
                    current_y = y if current_y is None else current_y
                else:
                    # finish previous line
                    current_line.sort(key=lambda x: x[0])  # left to right
                    lines.append([t[1] for t in current_line])

                    current_line = [(box[0], text)]
                    current_y = y

            # flush last line
            if current_line:
                current_line.sort(key=lambda x: x[0])
                lines.append([t[1] for t in current_line])

        return json.dumps({"result": True, "lines": lines})

    except Exception as e:
        raise e
        return json.dumps({"result": False, "error": f"Exception: {e}"})
# def main():
#     if len(sys.argv) != 2:
#         return False, json.dumps({"result": False, "error": "Usage: ocr.py <image_path>"})
#
#     try:
#         image_path = sys.argv[1]
#
#         result = ocr.predict(input=image_path)
#
#         out = []
#         for r in result:
#             out += r["rec_texts"]
#         return json.dumps({"result": True, "text": out})
#     except Exception as e:
#         return json.dumps({"result": False, "error": f"Exception: {e}"})


if __name__ == "__main__":
    print(main())
