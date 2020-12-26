import flask
import os
import random
from flask import send_file
from flask import flash, request, redirect, url_for
from flask_mail import Mail, Message
from werkzeug.utils import secure_filename


app = flask.Flask(__name__)
app.config["DEBUG"] = True

waiting_for_trigger = False
last_sent = None
colour = None
active = True

keys = []  # enter the token for your uploads, these have been removed for public viewing

UPLOAD_FOLDER = 'cdn/'
ALLOWED_EXTENSIONS = {'txt', 'pdf', 'png', 'jpg', 'jpeg', 'gif'}

# This part can be ignored if you dont want to be emailed whenever someone uploads something
# You'll need to remove a part later down the line which is commented out if you dont use the email
app.config['MAIL_SERVER'] = 'smtp.gmail.com'
app.config['MAIL_PORT'] = 465
app.config['MAIL_USERNAME'] = 'your email'
app.config['MAIL_PASSWORD'] = 'your password'
app.config['MAIL_USE_TLS'] = False
app.config['MAIL_USE_SSL'] = True

mail = Mail(app)


def allowed_file(filename):
    return '.' in filename and \
        filename.rsplit('.', 1)[1].lower() in ALLOWED_EXTENSIONS


@app.route('/')
def random_image():
    """
    Get a random image from our cdn!

    :return:
    """
    random_file = random.choice(os.listdir("cdn"))
    filename = f"cdn/{random_file}"
    return send_file(filename, mimetype='image/gif', as_attachment=False)


@app.route('/redirect')
def redirect(filename):
    """The redirect page after uploading a file"""
    global keys
    token = flask.request.args.get('token')
    filename_sanitized = str(filename).split("=")[-1]
    for key in keys:
        if token == key:
            return f'''
            File has uploaded! <a href="https://cdn.redpanda.pics/{filename_sanitized}">View here</a> 
            <br>
            Go back to uploading <a href="https://redpanda.pics/upload?token={token}">here</a>'''


@app.route('/upload', methods=["GET", "POST"])
def upload_file():
    """Upload a file to our cdn to be used in random images"""
    global keys
    token = flask.request.args.get('token')
    for key in keys:
        if token == key:
            if request.method == 'POST':
                if 'file' not in request.files:
                    flash('No file part')
                    return redirect(request.url)
                file = request.files['file']
                if file.filename == '':
                    flash('No selected file')
                    return redirect(request.url)
                if file and allowed_file(file.filename):
                    filename = secure_filename(file.filename)
                    file.save(os.path.join(app.config['UPLOAD_FOLDER'], filename))

                    # Again this is for mailing, you can remove it
                    msg = Message('Hello', sender='stomrparardiseonline@gmail.com', recipients=['julianjones663@gmail.com'])
                    msg.subject = f"New redpanda upload!"
                    msg.body = f"New upload from redpanda.pics!\n\nToken: {token}\nFile Name: {filename}\nURL: https://cdn.redpanda.pics/{filename}"
                    mail.send(msg)

                    return redirect(url_for('redirect', uploaded_file=filename))
            return '''
                <!doctype html>
                <title>Upload new File</title>
                <h1>Upload new File</h1>
                <form method=post enctype=multipart/form-data>
                  <input type=file name=file>
                  <input type=submit value=Upload>
                </form>
                '''


@app.route("/stats")
def stats():
    """Get information about the api"""
    json = {
        "total_images":  len(os.listdir("cdn")),
        "total_requests": "Work in progress",
        "endpoints": [
            {
                "url": "https://redpanda.pics/",
                "description": "Get a random image from our cdn!"
            },
            {
                "url": "https://redpanda.pics/upload?token=TOKEN",
                "description": "Upload a file to our cdn to be used in random images",
                "params": [
                    {
                        "name": "token",
                        "description": "the token to verify your identity, this is used to track if you didnt upload a wah"
                    }
                ]
            },
            {
                "url": "https://redpanda.pics/stats",
                "description": "Get information about the api"
            },
            {
                "url": "https://redpanda.pics/random",
                "description": "Get information about a random file and have it from our cdn"
            },
        ]
    }
    return json


@app.route("/random")
def random_api():
    """Get information about a random file and have it from our cdn"""
    random_file = random.choice(os.listdir("cdn"))
    file_type = random_file.split(".")[-1]
    json = {
        "url": f"https://cdn.redpanda.pics/{random_file}",
        "type": file_type
    }
    return json


app.config['UPLOAD_FOLDER'] = UPLOAD_FOLDER

# Running on local host because I proxy pass for nginx
app.run(port=2002, host='localhost')
