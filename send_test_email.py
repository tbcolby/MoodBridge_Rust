import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

# Email configuration
sender_email = "your-email@gmail.com"  # Replace with your email
receiver_email = "tbcolby@pm.me"
password = "your-password"  # Replace with your app password
smtp_host = "smtp.gmail.com"
smtp_port = 587

# Create the email
message = MIMEMultipart("alternative")
message["Subject"] = "🚀 WARP COMMAND Test Email"
message["From"] = sender_email
message["To"] = receiver_email

# Email body
html = '''
<html>
  <body>
    <h1>🚀 WARP COMMAND Test Email</h1>
    <p>Your system is operational and ready to analyze development patterns!</p>
    <ul>
      <li>Daily Log Analysis</li>
      <li>Development Insights</li>
      <li>MoodBridge Integration</li>
    </ul>
    <p>Test sent on: ''' + "2025-06-30" + '''</p>
  </body>
</html>
'''

# Attach the HTML part
part = MIMEText(html, "html")
message.attach(part)

# Send the email
try:
    with smtplib.SMTP(smtp_host, smtp_port) as server:
        server.starttls()
        server.login(sender_email, password)
        server.sendmail(sender_email, receiver_email, message.as_string())
    print("✅ Test email sent!")
except Exception as e:
    print(f"❌ Failed to send email: {e}")
