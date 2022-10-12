require 'rutie'
require 'base64'

def main
  Rutie.new(:procon_image, lib_path: './target/release').init 'Init_GetPoints', __dir__

  binary_data = File.read('./images/daruma_padd.png')
  encoded_img = Base64.strict_encode64(binary_data)

  ret = GetPoints.get_points(encoded_img)

  print ret, "\n"
end

def lambda_handler(event:, context:)
  Rutie.new(:procon_image, lib_path: './target/release').init 'Init_GetPoints', __dir__

  binary_data = File.read('./images/daruma_padd.png')
  encoded_img = Base64.strict_encode64(binary_data)

  GetPoints.get_points(encoded_img)
end

main
