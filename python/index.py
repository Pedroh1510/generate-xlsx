import mysql.connector
import xlsxwriter

mydb = mysql.connector.connect(
  host="localhost",
  user="root",
  password="mauFJcuf5dhRMQrjj",
  database="base"
)

mycursor = mydb.cursor()
mycursor.execute('SELECT count(1) FROM User2')
test = mycursor.fetchone()
total = test[0]
batch = 100
count = 1

workbook = xlsxwriter.Workbook('./demo.xlsx',{'constant_memory': True})
worksheet = workbook.add_worksheet()

def consulta_limitada(limit=batch,offset=0,q=0):
  mycursor.execute(f'SELECT * FROM User2 LIMIT {limit} OFFSET {offset}')
  myresult = mycursor.fetchall()
  for x in myresult:
    write_row(x,q)
    q = q +1
  return q

def write_row(row=(1),index=0):
  for col in range(0, len(row)):
        worksheet.write(index, col, row[col])

for index in range(0,total,batch):
  count = consulta_limitada(batch,index, count)
  print(index)
print(count)
workbook.close()